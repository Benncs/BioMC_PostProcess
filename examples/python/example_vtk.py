# import biomc_pp
# import matplotlib.pyplot as plt 
# import numpy as np 
# from typing import Tuple


# def normalize_concentration(
#     raw_concentration: np.ndarray, volumes: np.ndarray
# ) -> Tuple[np.ndarray, float, float]:
#     vtot = np.sum(volumes, axis=1)
#     mean_concentration = np.sum(raw_concentration * volumes, axis=1) / vtot
#     mean_concentration = mean_concentration.reshape(-1, 1)
#     variance = (
#         np.sum(np.power(raw_concentration - mean_concentration, 2) * volumes, axis=1)
#         / vtot
#     )
#     return raw_concentration / mean_concentration, mean_concentration, variance

# def plot_concentration_liquid(pp,time):
#     plt.figure(figsize=(10, 8))  # Set figure size

#     # First subplot (index 0)
#     plt.subplot(2, 2, 1)
#     S = pp.get_spatial_average_concentration(0, biomc_pp.Phase.Liquid)
#     plt.semilogy(time, S)
#     plt.title('Plot 1: Liquid Phase Glucose Concentration')

#     # Second subplot (index 1)
#     plt.subplot(2, 2, 2)
#     ctt = pp.get_spatial_average_concentration(1, biomc_pp.Phase.Liquid)
#     plt.plot(time, ctt)
#     plt.title('Plot 2: Liquid Phase O2 Concentration')

#     # Third subplot (index 2)
#     plt.subplot(2, 2, 3)
#     Ac = pp.get_spatial_average_concentration(2, biomc_pp.Phase.Liquid)
#     plt.plot(time, Ac)
#     plt.title('Plot 3: Liquid Phase Ac Concentration')

#     # Fourth subplot (index 3)
#     plt.subplot(2, 2, 4)
#     Co2 = pp.get_spatial_average_concentration(3, biomc_pp.Phase.Liquid)
#     plt.plot(time, Co2)
#     plt.title('Plot 4: Liquid Phase Co2 Concentration')

#     plt.tight_layout()  # Adjusts the layout so plots don't overlap
#     return S,Ac,Co2

# import cmtool.vtk
# name = "sanofi_poster_2"
# pp = biomc_pp.get_post_process(name,"/home-local/casale/Documents/thesis/code/BioCMA-MCST/results/")

# dest = f"/home-local/casale/Documents/thesis/code/BioCMA-MCST/results/{name}/vtk"
# vtu_path  = "/home-local/casale/Documents/thesis/cfd-cma/cma_data/sanofi/cma_mesh.vtu"




# time = biomc_pp.check_time_unit(pp)

# plot_concentration_liquid(pp,time)
# S = pp.get_spatial_average_concentration(0,biomc_pp.Phase.Liquid).reshape(-1,1)

# s = pp.get_concentrations(biomc_pp.Phase.Liquid)[:,:,0]

# ss = s/S

# # cmtool.vtk.mk_series(
# #             vtu_path,
# #             dest,
# #             name,
# #             time,
# #             ([ss,"normalized_concentration"]),
# #         )


# plt.figure()
# X = pp.get_biomass_concentration()
# print(X.shape)
# plt.plot(time,np.sum(X,axis=1))


# # print(ss.shape)
# # plt.plot(time,s/S)

# # plt.plot(time,X)
# # print(pp.get_property_names())
# plt.figure()
# spatial_property = pp.get_properties("mass",0)

# x0 = 1
# total_mass = np.sum(spatial_property)
# v = 299
# weigth = x0*v/total_mass
# print(weigth)
# X =np.array([np.sum(weigth*np.array(pp.get_properties("mass",i))) for i in range(pp.n_export)])/v

# plt.plot(time,X)
# n,c= pp.get_histogram(50,pp.n_export - 1, "length")
# n = n*1e6

# plt.figure()
# plt.bar(
#         n[:-1],
#         c[:-1],
#         width=np.diff(n),
#         edgecolor="black",
#         alpha=0.7,
#         color="blue",
#     )
# plt.show()

