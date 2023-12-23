import logging
from itertools import chain
from pathlib import Path
from dataclasses import dataclass
from typing import Optional, List, Tuple

from pydub import AudioSegment
from sqlalchemy.orm import Session
from sqlalchemy import select, MetaData, update
import sqlalchemy

from soundshouter.lib.settings import get_settings
from soundshouter.lib.db import Sound, Category, SubCategory


@dataclass
class FsSound:
    path: Path
    duration: float
    category: Optional[str]
    subcategory: Optional[str]


def get_categories(relative_path: Path) -> Tuple[Optional[str], Optional[str]]:
    match len(relative_path.parts):
        case 1:
            return None, None
        case 2:
            return relative_path.parts[0], None
        case _:
            return relative_path.parts[0], relative_path.parts[1]



def get_sound_list(path: Path) -> List[Sound]:
    sound_list = []
    suffix = ['mp3', 'wav', 'ogg']
    file_iter = chain(*[path.glob(f'**/*.{s}') for s in suffix])
    for file in file_iter:
        rel = file.relative_to(path)
        category, subcategory = get_categories(rel)

        try:
            sound = AudioSegment.from_file(file)
            sound_list.append(
                FsSound(
                    path=file,
                    duration=sound.duration_seconds,
                    category=category,
                    subcategory=subcategory
                )
            )
        except Exception as e:
            logging.warning(f"Could not open {file}")

    return sound_list


def import_sounds(path: Path):
    engine = sqlalchemy.create_engine(get_settings().db_url(_async=False))

    sounds = get_sound_list(path)
    create_entrys(engine, sounds)


def create_entrys(engine, sounds: List[Sound]):
    with Session(engine) as session:
        with session.begin():
            for snd in sounds:
                category = None
                if snd.category:
                    category = session.execute(
                        select(Category).filter_by(name=snd.category)
                    ).first()
                    if not category:
                        category = Category(name=snd.category)
                        session.add(category)
                    else:
                        category = category[0]

                subcategory = None
                if snd.subcategory:
                    subcat_query = select(SubCategory).filter_by(name=snd.subcategory)
                    subcategory = session.execute(subcat_query).first()
                    if not subcategory:
                        subcategory = SubCategory(name=snd.subcategory)
                        session.add(subcategory)
                    else:
                        subcategory = subcategory[0]

                sound_query = select(Sound).filter_by(path=str(snd.path.absolute()))
                sound = session.execute(sound_query).first()

                if not sound:
                    sound = Sound(
                        name=snd.path.stem,
                        path=str(snd.path.absolute()),
                        duration=snd.duration,
                        category=category,
                        subcategory=subcategory)
                    session.add(sound)

            session.commit()