import biomc_pp
import numpy as np
import matplotlib.pyplot as plt

if __name__=="__main__":
    pp = biomc_pp.get_post_process("cstr_0","/home-local/casale/Documents/thesis/simulations/ecoli_model_2024/out/steady_1/")
    time = biomc_pp.check_time_unit(pp)
    plt.figure()
    X = pp.get_biomass_concentration()
    plt.plot(time,X)
    plt.title("Biomass concentration")
    plt.savefig("./examples/out_py.png")


    n,c= pp.get_histogram(50,pp.n_export - 1, "nu_meta")
    n = n*3600
    print(len(c))
    print(len(c))
    plt.figure()
    plt.bar(
        n[:-1],
        c[:-1],
        width=np.diff(n),
        edgecolor="black",
        alpha=0.7,
        color="blue",
    )
    plt.show()