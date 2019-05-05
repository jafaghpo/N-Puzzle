if [ "$#" -eq 0 ]; then
    echo "You need to pass a file as parameter"
elif [ "$#" -eq 1 ]; then
    file_path=$1;

    echo "Misplaced tiles:" && ./npuzzle $file_path -h misplaced \
    && echo "\nOut of Axes:" && ./npuzzle $file_path -h axes \
    && echo "\nManhatan:" && ./npuzzle $file_path \
    && echo "\nLinear conflict:" && ./npuzzle $file_path -h conflict
else
    dir_path=$1;
    goal=$2;
    size=$3
    iter=$4;
    file_path="$1/$2_$3x$3_$4";

    echo "Misplaced tiles:" && ./npuzzle $dir_path -G $size -i $iter -e $goal -h misplaced \
    && echo "\nOut of Axes:" && ./npuzzle $file_path -h axes \
    && echo "\nManhatan:" && ./npuzzle $file_path \
    && echo "\nLinear conflict:" && ./npuzzle $file_path -h conflict
fi