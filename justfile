default:
        just --list

fetch day:
        mkdir -p input/day{{day}}
        curl -L -o input/day{{day}}/input https://adventofcode.com/2022/day/{{day}}/input 
