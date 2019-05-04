if [ "$#" -eq 0 ]; then
    echo "You need to pass a file as parameter"
elif [ "$#" -eq 1 ]; then
    file_path=$1;

    echo "A*:" && ./npuzzle $file_path -h conflict -a "A*" \
    && echo "\nIA*:" && ./npuzzle $file_path -h conflict -a "IA*" \
    && echo "\nILA*:" && ./npuzzle $file_path -h conflict -a "ILA*" \
    && echo "\nIDA*:" && ./npuzzle $file_path -h conflict -a "IDA*"
else
    dir_path=$1;
    goal=$2;
    size=$3
    iter=$4;
    file_path="$1/$2_$3x$3_$4";

    echo "A*:" && ./npuzzle $dir_path -G $size -i $iter -e $goal -h conflict -a "A*" \
    && echo "\nIA*:" && ./npuzzle $file_path -h conflict -a "IA*" \
    && echo "\nILA*:" && ./npuzzle $file_path -h conflict -a "ILA*" \
    && echo "\nIDA*:" && ./npuzzle $file_path -h conflict -a "IDA*"
fi