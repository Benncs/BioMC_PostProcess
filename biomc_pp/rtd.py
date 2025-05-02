import biomc_pp 
import numpy as np 
import matplotlib.pyplot as plt 

def get_rtd_from_scalar(pp,time,step_concentration):

    c = pp.get_spatial_average_concentration(0,biomc_pp.Phase.Liquid)
    c0 = c[0]
    delta_c = c - c0
    step_concentration = 5

    # Cumulative probability 
    f_rtd = delta_c / step_concentration  
    # By defintion F(t)=integral 0 to t (E(t))
    e_rtd = np.diff(f_rtd) / np.diff(time)

    return f_rtd,e_rtd

def get_rtd_particle(pp):
    probes = pp.get_probes()/3600
    return np.histogram(probes, bins=100,density=True)

def plot_rtd(pp,time,step_concentration):
    
    f_rtd,e_rtd = get_rtd_from_scalar(pp,time,step_concentration)
    plt.figure()
    plt.plot(time[1:],e_rtd,'--',color='red',label='Scalar')
    c,e = get_rtd_particle(pp)
    plt.bar(
        e[:-1],
        c,
        width=np.diff(e),
        edgecolor="black",
        alpha=0.7,
        color="blue",
        label="Particles"
    )
    return f_rtd,e_rtd

