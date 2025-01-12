import biomc_pp
import matplotlib.pyplot as plt 
import numpy as np

import timeit
def test():
    pp = biomc_pp.PostProcess("cstr","/home/benjamin/Documents/code/cpp/BioCMA-MCST/results/")
    return pp.get_number_particle()

def main():
    

    # results = biomc_pp.PostProcess("cstr","/home/benjamin/Documents/code/cpp/BioCMA-MCST/results/")
    # c =results.get_spatial_average_concentration(0,biomc_pp.Phase.Liquid)
    # ctt =results.get_time_average_concentration(0,0,biomc_pp.Phase.Liquid)
    
    pp = biomc_pp.PostProcess("cstr2","/home/benjamin/Documents/code/cpp/BioCMA-MCST/results/")

    p = pp.get_growth_in_number()
    
    # cx = pp.get_biomass_concentration()
    # plt.plot(pp.time,cx)

    n_export = pp.n_export

    def calculate_mean():
        mean = np.zeros((n_export,))
        for i in range(n_export):
            mean[i] = pp.get_population_mean("age", i)
        return mean

    def calculate_mean2():
        return pp.get_time_population_mean("age")

    mean_time = timeit.timeit(calculate_mean, number=10)
    print(f"Mean execution time: {mean_time / 10} seconds")

    # Benchmark mean2
    mean2_time = timeit.timeit(calculate_mean2, number=10)
    print(f"Mean2 execution time: {mean2_time / 10} seconds")

    plt.plot(calculate_mean())
    mean =pp.get_time_population_mean("age")
    plt.plot(mean,"*")
    plt.show()

    # print(p1)
    # print(ctt)
    # plt.plot(results.time,p)
    # plt.show()






main()