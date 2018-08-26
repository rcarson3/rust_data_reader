# rust_data_reader

An attempt at bringing into Rust data file readers that are similar in scope to those offered by the numpy package in python for genfromtxt and loadtxt. It currently is pretty rough and should not used by anyone. It is bound to be slow. The erro handling is okay as of right now but it could be better. A vast number of edge cases still need to be tested. It also has currently only been examined for type data of int32. 

The code is very much in a pre-alpha state currently. Once all of the primitative types have been added. The find comment lines has improved then one might start to be able to use on data files without missing data. Data that is missing the option type will be used to wrap the data.

