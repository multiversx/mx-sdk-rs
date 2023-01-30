cd contracts/benchmarks/mappers
run-scenarios . > ../../../tools/extract-benchmarks/bench.log
cd ../../..
cd tools/extract-benchmarks
./extract.py
cd ../..
