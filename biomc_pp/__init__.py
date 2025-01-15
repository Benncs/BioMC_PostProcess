
from biomc_pp import biomc_pp
import numpy as np
FIGURE_TYPE=".png"
TIME_UNIT ="s"

def set_time_unit_to_hour():
    global TIME_UNIT
    TIME_UNIT="h"

def get_time_unit()->str:
    return TIME_UNIT


def check_time_unit(results: biomc_pp.PostProcess)->np.ndarray:
    # Conversion to hour if duration too long
    if results.time[-1] > 10000:
        t = np.array(results.time) / 3600.0
        set_time_unit_to_hour()
        return t
    else:
        return np.array(results.time)


def get_post_process(name:str,root:str="./results"):
    return biomc_pp.PostProcess(name,root)
