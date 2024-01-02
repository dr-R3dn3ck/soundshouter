from functools import lru_cache
from typing import Union, List

from sqlalchemy import select, update
import asyncio
from concurrent.futures import ThreadPoolExecutor
from fastapi import FastAPI, Depends, HTTPException
from pydub import AudioSegment
from pydub.playback import play
from sqlalchemy.ext.asyncio import create_async_engine, async_sessionmaker
from sqlalchemy.orm import Session
from faststream.rabbit.fastapi import RabbitRouter

from soundshouter.lib.settings import get_settings
from soundshouter.lib.db import Sound, Category, SubCategory
from soundshouter.lib import api_models as models

router = RabbitRouter(
    get_settings().rabbitmq_url,
    username=get_settings().rabbitmq_username,
    password=get_settings().rabbitmq_password,
    max_consumers=get_settings().max_consumers
)
app = FastAPI(
    title="SoundShouter",
    description="",
    version="0.1.0",
    redoc_url=None,
    lifespan=router.lifespan_context)
app.include_router(router)

engine = create_async_engine(get_settings().db_url())
SessionLocal = async_sessionmaker(engine, autocommit=False, autoflush=False)


async def get_db():
    db = SessionLocal()
    try:
        yield db
    finally:
        await db.close()


def _play(path: str):
    sound = AudioSegment.from_file(path)
    play(sound)


async def play_async(path: str):
    with ThreadPoolExecutor() as pool:
        await asyncio.get_event_loop().run_in_executor(pool, _play, path)


@router.subscriber("soundshouter")
@router.publisher("soundshouter-playlist")
async def queue_play_sound(sound_id: int, db=Depends(get_db)):
    """Receive rabbitmq messages and play a sound"""
    query = select(Sound).filter_by(id=sound_id)
    result = await db.execute(query)
    db_sound = result.first()[0]

    if not db_sound:
        return f"Sound {sound_id} not found"

    await play_async(db_sound.path)
    return f"playing {db_sound.name}"


@app.put("/play/{sound_id}")
async def play_sound(sound_id: int, db: Session = Depends(get_db)):
    """adds a message to the rabbitmq queue"""
    query = select(Sound).filter_by(id=sound_id)
    result = await db.execute(query)

    if not result:
        return HTTPException(status_code=404, detail="Sound not found")

    db_sound = result.first()[0]
    await router.broker.publish(db_sound.id, "soundshouter")

    db_sound.play_count += 1
    update_q = update(Sound) \
        .filter_by(id=sound_id) \
        .values(play_count=db_sound.play_count + 1)

    await db.execute(update_q)
    return {}


@app.get("/sounds", response_model=List[models.Sound])
async def read_sounds(limit: Union[int, None] = 10, offset: Union[int, None] = None, db: Session = Depends(get_db)):
    query = select(Sound).offset(offset).limit(limit)
    result = await db.execute(query)
    sounds = [r[0] for r in result.all()]
    return sounds


@app.get("/categories", response_model=List[models.Category])
async def read_categories(limit: Union[int, None] = 10, offset: Union[int, None] = None, db: Session = Depends(get_db)):
    query = select(Category).offset(offset).limit(limit)
    result = await db.execute(query)
    categories = [r[0] for r in result.all()]
    return categories


@app.get("/subcategories", response_model=List[models.Subcategory])
async def read_subcategories(limit: Union[int, None] = 10, offset: Union[int, None] = None,
                             db: Session = Depends(get_db)):
    query = select(SubCategory).offset(offset).limit(limit)
    result = await db.execute(query)
    subcategories = [r[0] for r in result.all()]
    return subcategories
