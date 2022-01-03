cd contracts/benchmarks/mappers
mandos-test . > ../../../tools/extract-benchmarks/bench.log
cd ../../..
cd tools/extract-benchmarks
./extract.py
cd ../..
