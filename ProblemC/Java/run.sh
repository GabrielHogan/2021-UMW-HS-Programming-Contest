#!/bin/bash

echo "Attempting to run ProblemC.java"
echo "Removing old compilations"
rm -r *.class
echo "Compiling..."
javac *.java
echo "Running:"
java ProblemC