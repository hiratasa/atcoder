CXXFLAGS = -std=c++17 -Wall

%: %.cpp
	g++ $(CXXFLAGS) $< -o $@

%.d: %.cpp
	g++ $(CXXFLAGS) -g $< -o $@

.PHONY: FORCE
FORCE:

.PRECIOUS: %.d

%.debug: %.d FORCE
	gdb $<