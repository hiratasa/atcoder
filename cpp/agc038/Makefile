%: %.cpp
	g++ -std=c++17 -O3 $^ -o $@

%.debug: %.cpp
	g++ -std=c++17 -g $^ -o $@
	gdb $@
