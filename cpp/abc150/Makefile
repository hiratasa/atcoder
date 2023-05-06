%: %.cpp
	g++ -std=c++14 -Wall $^ -o $@

%.debug: %.cpp
	g++ -std=c++14 -g $^ -o $@
	gdb $@