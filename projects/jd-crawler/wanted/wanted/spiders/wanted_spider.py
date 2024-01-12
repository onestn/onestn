import scrapy


class WantedSpider(scrapy.Spider):
    name='wanted'

    def start_requests(self):
        start_urls = [
            'https://www.wanted.co.kr/wdlist/518/655?country=kr&job_sort=job.latest_order&locations=all',
        ]

    def parse(self, response):
        pass
