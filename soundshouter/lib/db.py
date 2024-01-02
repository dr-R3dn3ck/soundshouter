from typing import List
from sqlalchemy import ForeignKey, String, Float, UniqueConstraint
from sqlalchemy import create_engine
from sqlalchemy.orm import DeclarativeBase, Mapped, mapped_column, relationship

from soundshouter.lib.settings import get_settings


def init_schema():
    engine = create_engine(get_settings().db_url(_async=False))
    Base.metadata.create_all(engine)


def truncate_db():
    engine = create_engine(get_settings().db_url(_async=False))
    Base.metadata.drop_all(engine)


class Base(DeclarativeBase):
    pass


class Sound(Base):
    __tablename__ = 'sound'

    id: Mapped[int] = mapped_column(primary_key=True)
    name: Mapped[str] = mapped_column(String(50))
    path: Mapped[str] = mapped_column(String(400), unique=True)
    duration: Mapped[float] = mapped_column(Float)
    play_count: Mapped[int] = mapped_column(default=0)
    category_id: Mapped[int] = mapped_column(ForeignKey("category.id"), nullable=True)
    subcategory_id: Mapped[int] = mapped_column(ForeignKey("subcategory.id"), nullable=True)

    category: Mapped["Category"] = relationship(back_populates="sound")
    subcategory: Mapped["SubCategory"] = relationship(back_populates="sound")

    def __repr__(self):
        return f"<Sound(id={self.id} name={self.name}, duration={self.duration})>"


class Category(Base):
    __tablename__ = 'category'

    id: Mapped[int] = mapped_column(primary_key=True)
    name: Mapped[str] = mapped_column(String(50), unique=True)

    sound: Mapped[List["Sound"]] = relationship(back_populates="category")

    def __repr__(self):
        return f"<Category(id={self.id} name={self.name})>"


class SubCategory(Base):
    __tablename__ = 'subcategory'
    __table_args__ = (
        UniqueConstraint("category_id", "name"),
    )

    id: Mapped[int] = mapped_column(primary_key=True)
    category_id: Mapped[int] = mapped_column(ForeignKey("category.id"))
    name: Mapped[str] = mapped_column(String(50), unique=True)

    sound: Mapped[List["Sound"]] = relationship(back_populates="subcategory")
    category: Mapped["Category"] = relationship()

    def __repr__(self):
        return f"<SubCategory(id={self.id} name={self.name})>"
