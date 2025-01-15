

from .biomc_pp import *
import numpy as np
FIGURE_TYPE=".png"
TIME_UNIT ="s"

__all__ = []
__doc__ = biomc_pp.__doc__
if hasattr(biomc_pp, "__all__"):
    __all__ = biomc_pp.__all__


def set_time_unit_to_hour():
    global TIME_UNIT
    TIME_UNIT="h"

def get_time_unit()->str:
    return TIME_UNIT


def check_time_unit(results: PostProcess)->np.ndarray:
    # Conversion to hour if duration too long
    if results.time[-1] > 10000:
        t = np.array(results.time) / 3600.0
        set_time_unit_to_hour()
        return t
    else:
        return np.array(results.time)


def get_post_process(name:str,root:str="./results"):
    return PostProcess(name,root)


__all__.extend([
    "set_time_unit_to_hour",
    "get_time_unit",
    "check_time_unit",
    "get_post_process",
])
