from typing import List, Optional, Union
from datetime import datetime 

from pydantic import BaseModel, Field


class Movie(BaseModel):
    mid: int
    genre: str
    rate: Union[int, float]
    tag: Optional[str] = None
    date: Optional[datetime] = None
    some_variable_list: List[int] = []


class User(BaseModel):
    """
    gt: 값보다 큰 
    ge: 값보다 크거나 같은
    lt: 값보다 작은
    le: 값보다 작거나 같은 
    """
    uid: int
    name: str = Field(min_length=2, max_length=10)
    age: int = Field(gt=1, le=130)

tmp_movie_data = {
    'mid': 1,
    'genre': 'action',
    'rate': 1.5,
    'tag': None,
    'date': '2023-01-03 19:12:00'
}

tmp_user_data = {
    'uid': 100,
    'name': 'onestone',
    'age': 27
}

tmp_movie = Movie(**tmp_movie_data)
tmp_user_data = User(**tmp_user_data)

print(tmp_movie)
print(tmp_user_data)
