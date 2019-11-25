echo "Building all days"
for i in aoc*
do
  cargo build --manifest-path=$i/Cargo.toml --release
done

for i in aoc*
do
  echo "Running $i"
  $i/target/release/$i < $i/input/input.txt
done
