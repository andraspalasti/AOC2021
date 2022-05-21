#!/usr/bin/env bash

solution=$1
if [ "$solution" = "" ]; then
	echo "You have to provide a solution"
	exit 1
fi

ls src | tr \[:blank:\] "\n" | grep -qe "^$solution\$"
if [ $? -eq 0 ]; then
	echo "A solution with name '$solution' already exists"
	exit 1
fi

mkdir "./src/$solution"
echo "Created folder './src/$solution'"

echo "pub fn main() {
	println!(\"Solution for: $solution\");
}" >> "./src/$solution/mod.rs"
echo "Created file './src/$solution/mod.rs'"


