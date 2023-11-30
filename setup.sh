#/bin/bash

setup() {
    for i in `seq 1 25`; do
    	local dir="day${i}";
    	cargo new $dir;
    	cd $dir;
    	rm -rf ./.git
    	mkdir src/bin;
    	rm src/main.rs;
    	cp ../template.rs src/bin/part1.rs;
    	cp ../template.rs src/bin/part2.rs;
    	echo -n "placeholder" > src/bin/test.txt;
    	echo -n "placeholder" > src/bin/input.txt;
    	cd -;
    done
}

cleanup() {
    rm -rf ./day*;
}

if [ $# != 1 ]; then
    >&2 echo "args: setup | clean";
    exit 1;
fi

if [ $1 == "setup" ]; then
    setup;
elif [ $1 == "clean" ]; then
    cleanup;
fi
