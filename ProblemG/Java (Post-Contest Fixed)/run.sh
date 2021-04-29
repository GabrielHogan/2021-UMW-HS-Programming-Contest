#!/bin/bash

echo "Attempting to run ProblemG.java"
echo "Removing old compilations"
rm -r *.class
echo "Compiling..."
javac *.java
echo "Running:"
java ProblemG