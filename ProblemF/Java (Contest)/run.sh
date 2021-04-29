#!/bin/bash

echo "Attempting to run ProblemF.java"
echo "Removing old compilations"
rm -r *.class
echo "Compiling..."
javac *.java
echo "Running:"
java ProblemF