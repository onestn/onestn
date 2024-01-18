# Scrapy
Scrapy는 크롤링/스크레이핑을 위한 파이썬 프레임워크입니다. 풍부한 기능이 제공되므로 사용자는 페이지에서 데이터를 추출하는 본질적인 작업에만 집중할 수 있습니다.
 
- Scrapy의 대표 기능
    - 웹 페이지에서 링크 추출하기
    - robots.txt를 기반으로 허가된 페이지와 금지된 페이지 구분하기
    - XML 사이트맵 추출과 링크 추출하기
    - 도메인과 IP 주소마다 크롤링 시간 간격 조정하기
    - 여러 개의 크롤링 대상을 병렬 처리하기
    - 중복된 URL 크롤링하지 않기
    - 오류가 발생했을 때 특정 횟수만큼 재시도하기
    - 크롤러를 데몬으로 만들기와 잡 관리하기
 
Scrpy는 Event Driven 네트워크 프로그래밍 엔진인 Twisted를 기반으로 만들어졌습니다. 따라서 웹사이트 다운로드 처리를 비동기적으로 실행하므로 다운로드 중에도 스크레이핑 처리 등을 할 수 있습니다. (Scrapy처럼 스크레이핑을 위한 프레임워크는 다른 프로그래밍 언어에서 거의 찾아볼 수 없다.)
 
## Primary Component - Spider
Scrapy를 사용하면 주로 Spider라는 이름의 클래스를 만들게 됩니다. 대상 웹사이트마다 Spider를 만들며, 이러한 Spider 클래스에 크롤링 처리와 스크레이핑 처리를 작성합니다.
 
Scrapy는 프로젝트 단위이며, 여러 개의 Spider 관련 클래스를 통합 관리할 수 있습니다. 일회용 Spider를 만드는 것이 아니라면 프로젝트를 사용하는 것이 기본입니다.
 
## File Structure
```
├── scrapy.cfg              # Crawler Deploy 관련 설정 파일
└── tutorial
   ├── __init__.py          # 프로젝트는 파이썬 모듈이므로.
   ├── __pycache__
   ├── items.py             # Item 정의 파일
   ├── middlewares.py 
   ├── pipelines.py         # Item Pipeline 정의 파일
   ├── settings.py          # 프로젝트 설정 파일
   └── spiders              # Spider를 저장하는 디렉터리
      ├── __init__.py
      ├── __pycache__
      └── custom_spider.py  # User Define Spider
```
> settings.py에 `DOWNLOAD_DELAY = 1`을 추가하여 의도치 않게 웹사이트에 부하를 주지 않게 꼭 설정한다.
 
## Primary Component - Item
Item은 Spider가 추출한 데이터를 저장할 객체입니다. Spider처럼 추출한 데이터를 저장하기 위해 dict를 사용해도 괜찮지만, Item을 사용하면 다음과 같은 장점이 있습니다.
    1. 여러 종류의 데이터를 추출했을 때 클래스를 기반으로 객체를 판별할 수 있음
    2. 미리 정의한 필드에 데이터를 입력하므로, 필드 이름을 잘못 적는 실수를 줄일 수 있음
    3. 메서드를 정의할 수 있음
 
Item은 프로젝트의 items.py에서 정의합니다. Item 클래스는 `scrapy.Item`을 상속받으며, 필드를 `field_name = scrapy.Field()`라는 형태로 정의합여 사용합니다.
 
```python
class Headline(scrapy.Item):
    title = scrapy.Field()
    body = scrapy.Field()
```
 
Item 객체는 dict처럼 키를 지정하여 값에 접근합니다.
```python
item = Headline()
item['title'] = 'Example'
```
