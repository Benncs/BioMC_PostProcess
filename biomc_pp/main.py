import biomc_pp
import matplotlib.pyplot as plt 



def test():
    pp = biomc_pp.PostProcess("cstr","/home/benjamin/Documents/code/cpp/BioCMA-MCST/results/")
    return pp.get_number_particle()

def main():
    

    # results = biomc_pp.PostProcess("cstr","/home/benjamin/Documents/code/cpp/BioCMA-MCST/results/")
    # c =results.get_spatial_average_concentration(0,biomc_pp.Phase.Liquid)
    # ctt =results.get_time_average_concentration(0,0,biomc_pp.Phase.Liquid)

    pp = biomc_pp.PostProcess("cstr","/home/benjamin/Documents/code/cpp/BioCMA-MCST/results/")
    p = pp.get_growth_in_number()
    
    cx = pp.get_biomass_concentration()
    plt.plot(pp.time,cx)


    # print(p1)
    # print(ctt)
    # plt.plot(results.time,p)
    plt.show()


main()