from typing import Optional

from pydantic import BaseModel


class Sound(BaseModel):
    id: int
    name: str
    # path: str
    duration: float
    play_count: int
    category_id: int
    subcategory_id: Optional[int]
    # category: str
    # subcategory: str

    class Config:
        from_attributes = True


class Category(BaseModel):
    id: int
    name: str

    class Config:
        from_attributes = True


class Subcategory(BaseModel):
    id: int
    name: str
    category_id: Optional[int]

    class Config:
        from_attributes = True
