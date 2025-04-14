from __future__ import annotations

import ast
import warnings
from datetime import timedelta
from functools import cached_property
from typing import TYPE_CHECKING, Any, Sequence
from uuid import uuid4

from airflow.configuration import conf
from airflow.exceptions import AirflowException, AirflowProviderDeprecationWarning
from airflow.models import BaseOperator
from airflow.providers.amazon.aws.hooks.emr import EmrContainerHook, EmrHook, EmrServerlessHook
from airflow.providers.amazon.aws.links.emr import (
    EmrClusterLink,
    EmrLogsLink,
    EmrServerlessCloudWatchLogsLink,
    EmrServerlessDashboardLink,
    EmrServerlessLogsLink,
    EmrServerlessS3LogsLink,
    get_log_uri,
)
from airflow.providers.amazon.aws.triggers.emr import (
    EmrAddStepsTrigger,
    EmrContainerTrigger,
    EmrCreateJobFlowTrigger,
    EmrServerlessCancelJobsTrigger,
    EmrServerlessCreateApplicationTrigger,
    EmrServerlessDeleteApplicationTrigger,
    EmrServerlessStartApplicationTrigger,
    EmrServerlessStartJobTrigger,
    EmrServerlessStopApplicationTrigger,
    EmrTerminateJobFlowTrigger,
)
from airflow.providers.amazon.aws.utils import validate_execute_complete_event
from airflow.providers.amazon.aws.utils.waiter import waiter
from airflow.providers.amazon.aws.utils.waiter_with_logging import wait
from airflow.utils.helpers import exactly_one, prune_dict
from airflow.utils.types import NOTSET, ArgNotSet

if TYPE_CHECKING:
    from airflow.utils.context import Context


class EmrServerlessCreateApplicationOperator(BaseOperator):
    """
    Operator to create Serverless EMR Application.

    .. seealso::
        For more information on how to use this operator, take a look at the guide:
        :ref:`howto/operator:EmrServerlessCreateApplicationOperator`

    :param release_label: The EMR release version associated with the application.
    :param job_type: The type of application you want to start, such as Spark or Hive.
    :param wait_for_completion: If true, wait for the Application to start before returning. Default to True.
        If set to False, ``waiter_max_attempts`` and ``waiter_delay`` will only be applied when
        waiting for the application to be in the ``CREATED`` state.
    :param client_request_token: The client idempotency token of the application to create.
      Its value must be unique for each request.
    :param config: Optional dictionary for arbitrary parameters to the boto API create_application call.
    :param aws_conn_id: The Airflow connection used for AWS credentials.
        If this is None or empty then the default boto3 behaviour is used. If
        running Airflow in a distributed manner and aws_conn_id is None or
        empty, then default boto3 configuration would be used (and must be
        maintained on each worker node).
    :param waiter_countdown: (deprecated) Total amount of time, in seconds, the operator will wait for
        the application to start. Defaults to 25 minutes.
    :param waiter_check_interval_seconds: (deprecated) Number of seconds between polling the state
        of the application. Defaults to 60 seconds.
    :waiter_max_attempts: Number of times the waiter should poll the application to check the state.
        If not set, the waiter will use its default value.
    :param waiter_delay: Number of seconds between polling the state of the application.
    :param deferrable: If True, the operator will wait asynchronously for application to be created.
        This implies waiting for completion. This mode requires aiobotocore module to be installed.
        (default: False, but can be overridden in config file by setting default_deferrable to True)
    """

    def __init__(
        self,
        release_label: str,
        job_type: str,
        client_request_token: str = "",
        config: dict | None = None,
        wait_for_completion: bool = True,
        aws_conn_id: str | None = "aws_default",
        waiter_countdown: int | ArgNotSet = NOTSET,
        waiter_check_interval_seconds: int | ArgNotSet = NOTSET,
        waiter_max_attempts: int | ArgNotSet = NOTSET,
        waiter_delay: int | ArgNotSet = NOTSET,
        deferrable: bool = conf.getboolean("operators", "default_deferrable", fallback=False),
        **kwargs,
    ):
        if waiter_check_interval_seconds is NOTSET:
            waiter_delay = 60 if waiter_delay is NOTSET else waiter_delay
        else:
            waiter_delay = waiter_check_interval_seconds if waiter_delay is NOTSET else waiter_delay
            warnings.warn(
                "The parameter waiter_check_interval_seconds has been deprecated to standardize "
                "naming conventions.  Please use waiter_delay instead.  In the "
                "future this will default to None and defer to the waiter's default value.",
                AirflowProviderDeprecationWarning,
                stacklevel=2,
            )
        if waiter_countdown is NOTSET:
            waiter_max_attempts = 25 if waiter_max_attempts is NOTSET else waiter_max_attempts
        else:
            if waiter_max_attempts is NOTSET:
                # ignoring mypy because it doesn't like ArgNotSet as an operand, but neither variables
                # are of type ArgNotSet at this point.
                waiter_max_attempts = waiter_countdown // waiter_delay  # type: ignore[operator]
            warnings.warn(
                "The parameter waiter_countdown has been deprecated to standardize "
                "naming conventions.  Please use waiter_max_attempts instead. In the "
                "future this will default to None and defer to the waiter's default value.",
                AirflowProviderDeprecationWarning,
                stacklevel=2,
            )
        self.aws_conn_id = aws_conn_id
        self.release_label = release_label
        self.job_type = job_type
        self.wait_for_completion = wait_for_completion
        self.kwargs = kwargs
        self.config = config or {}
        self.waiter_max_attempts = int(waiter_max_attempts)  # type: ignore[arg-type]
        self.waiter_delay = int(waiter_delay)  # type: ignore[arg-type]
        self.deferrable = deferrable
        super().__init__(**kwargs)

        self.client_request_token = client_request_token or str(uuid4())

    @cached_property
    def hook(self) -> EmrServerlessHook:
        """Create and return an EmrServerlessHook."""
        return EmrServerlessHook(aws_conn_id=self.aws_conn_id)

    def execute(self, context: Context) -> str | None:
        response = self.hook.conn.create_application(
            clientToken=self.client_request_token,
            releaseLabel=self.release_label,
            type=self.job_type,
            **self.config,
        )
        application_id = response["applicationId"]

        if response["ResponseMetadata"]["HTTPStatusCode"] != 200:
            raise AirflowException(f"Application Creation failed: {response}")

        self.log.info("EMR serverless application created: %s", application_id)
        if self.deferrable:
            self.defer(
                trigger=EmrServerlessCreateApplicationTrigger(
                    application_id=application_id,
                    aws_conn_id=self.aws_conn_id,
                    waiter_delay=self.waiter_delay,
                    waiter_max_attempts=self.waiter_max_attempts,
                ),
                timeout=timedelta(seconds=self.waiter_max_attempts * self.waiter_delay),
                method_name="start_application_deferred",
            )

        waiter = self.hook.get_waiter("serverless_app_created")
        wait(
            waiter=waiter,
            waiter_delay=self.waiter_delay,
            waiter_max_attempts=self.waiter_max_attempts,
            args={"applicationId": application_id},
            failure_message="Serverless Application creation failed",
            status_message="Serverless Application status is",
            status_args=["application.state", "application.stateDetails"],
        )
        self.log.info("Starting application %s", application_id)
        self.hook.conn.start_application(applicationId=application_id)

        if self.wait_for_completion:
            waiter = self.hook.get_waiter("serverless_app_started")
            wait(
                waiter=waiter,
                waiter_max_attempts=self.waiter_max_attempts,
                waiter_delay=self.waiter_delay,
                args={"applicationId": application_id},
                failure_message="Serverless Application failed to start",
                status_message="Serverless Application status is",
                status_args=["application.state", "application.stateDetails"],
            )
        return application_id

    def start_application_deferred(self, context: Context, event: dict[str, Any] | None = None) -> None:
        if event is None:
            self.log.error("Trigger error: event is None")
            raise AirflowException("Trigger error: event is None")
        elif event["status"] != "success":
            raise AirflowException(f"Application {event['application_id']} failed to create")
        self.log.info("Starting application %s", event["application_id"])
        self.hook.conn.start_application(applicationId=event["application_id"])
        self.defer(
            trigger=EmrServerlessStartApplicationTrigger(
                application_id=event["application_id"],
                aws_conn_id=self.aws_conn_id,
                waiter_delay=self.waiter_delay,
                waiter_max_attempts=self.waiter_max_attempts,
            ),
            timeout=timedelta(seconds=self.waiter_max_attempts * self.waiter_delay),
            method_name="execute_complete",
        )

    def execute_complete(self, context: Context, event: dict[str, Any] | None = None) -> None:
        event = validate_execute_complete_event(event)

        if event["status"] != "success":
            raise AirflowException(f"Trigger error: Application failed to start, event is {event}")

        self.log.info("Application %s started", event["application_id"])
        return event["application_id"]
