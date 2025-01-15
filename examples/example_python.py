import biomc_pp
import numpy as np
import matplotlib.pyplot as plt

if __name__=="__main__":
    pp = biomc_pp.get_post_process("cstr","/home-local/casale/Documents/thesis/simulations/ecoli_model_2024/out")
    time = biomc_pp.check_time_unit(pp)
    plt.figure()
    X = pp.get_biomass_concentration()
    plt.plot(time,X)
    plt.title("Biomass concentration")
    plt.savefig("./examples/out_py.png")
