%: %.cpp
	g++ -std=c++17 -Wall $< -o $@

%.d: %.cpp
	g++ -std=c++17 -g $< -o $@

.PHONY: FORCE
FORCE:

.PRECIOUS: %.d

%.debug: %.d FORCE
	gdb $<