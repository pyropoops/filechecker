if [ $# == 0 ] 
then
  echo "Usage - ./run.sh <path>"
else
  cargo run $@ > output.txt
fi