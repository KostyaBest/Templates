g++ -c -I /usr/lib/jvm/java-1.11.0-openjdk-amd64/include/ -I /usr/lib/jvm/java-11-openjdk-amd64/include/linux test1_Test1.cpp &&
g++ -shared -fPIC -o libtest1_Test1.so test1_Test1.o -lc &&
echo "Done"
