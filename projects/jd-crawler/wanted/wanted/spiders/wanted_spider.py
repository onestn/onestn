import scrapy


# TODO: Change url in Request()
class WantedSpider(scrapy.Spider):
    name='wanted'

    def start_requests(self):
        # GET request
        yield scrapy.Request(
            "https://httpbin.org/get", 
            meta={
                "playwright": True
            })
        # POST request
        yield scrapy.FormRequest(
            url="https://httpbin.org/post",
            formdata={"foo": "bar"},
            meta={"playwright": True},
        )

    def parse(self, response, **kwargs):
        # 'response' contains the page as seen by the browser
        return {"url": response.url}
