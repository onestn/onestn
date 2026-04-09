**`s3deploy` CLI 기획안**

---

**사용법:**
```
s3deploy <spark|airflow> <filepath>
```

**동작:**
1. 첫 번째 인자로 배포 대상 구분 (`spark` / `airflow`)
2. 두 번째 인자로 파일 경로 받기
3. 파일 경로에서 부모폴더명 + 파일명 추출
4. 대상에 따라 환경변수 읽기:
   - `spark` → `$S3_EMR`
   - `airflow` → `$S3_MWAA`
5. S3 경로 조합: `{환경변수}sync-data/{부모폴더}/{파일명}`
6. `aws s3 cp` 실행

**예시:**
```
s3deploy spark /home/user/project/utils/helper.py
→ aws s3 cp /home/user/project/utils/helper.py s3://emr-bucket/prefix/sync-data/utils/helper.py

s3deploy airflow /home/user/project/dags/my_dag.py
→ aws s3 cp /home/user/project/dags/my_dag.py s3://mwaa-bucket/prefix/sync-data/dags/my_dag.py
```

**빌드:**
```
rustc s3deploy.rs -o s3deploy
```
컴파일 후 바이너리를 PATH에 배치

**Neovim 키맵:**
- `<leader>sds` → `:!s3deploy spark %`
- `<leader>sda` → `:!s3deploy airflow %`

**구현 힌트 (순서대로):**
1. `std::env::args()` — 인자 파싱
2. `std::path::Path` — `parent()` → `file_name()`으로 폴더명, `file_name()`으로 파일명
3. `std::env::var()` — 환경변수 읽기
4. `format!()` — 경로 조합
5. `std::process::Command` — aws cli 호출
6. `status.success()` — 결과 처리

