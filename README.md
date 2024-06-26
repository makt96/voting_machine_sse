# Voting Machine Project with Backdoors

## Project Overview

The goal of this project is to utilize the knowledge from the course to write a voting machine in Rust and embed backdoors inside of it that would allow a knowledgeable attacker to cheat in the election. Then, we would be given another team’s voting machine to attack, attempting to find all of the vulnerabilities (intentional or not) in their system. Finally, our group will present on both our machine and our experience attacking the other group’s machine.

## Learning Objectives

- **Voting Machine Completeness & Correctness**: Ensure the voting machine is fully functional and accurately records votes.
- **Voting Machine and Backdoor Design & Correctness**: Design the voting machine with intentional backdoors that can be exploited, and ensure they function as intended.
- **Red Team Voting Machine Analysis & Vulnerability Discussion**: Analyze another team's voting machine to identify vulnerabilities and discuss potential exploits.
- **Red Team Backdoor Exploitation**: Exploit the identified backdoors and vulnerabilities in the other team’s voting machine.
- **Unit Testing of Program Functions and Components**: Perform unit testing to validate the correctness and reliability of the voting machine components.

## Project Steps

1. **Voting Machine Development**
   - Develop a voting machine using Rust, ensuring basic functionality and accuracy in vote recording.
   - Embed intentional backdoors in the system to allow a knowledgeable attacker to manipulate the results.

2. **Backdoor Implementation**
   - Design and implement backdoors that can be triggered under specific conditions.
   - Ensure the backdoors are concealed and can be activated without detection during normal use.

3. **Red Team Analysis**
   - Receive and analyze another team’s voting machine.
   - Identify both intentional and unintentional vulnerabilities in their system.
   - Document the potential exploits and vulnerabilities found.

4. **Backdoor Exploitation**
   - Develop methods to exploit the identified backdoors in the other team’s voting machine.
   - Test the exploitation techniques to ensure they work as intended.

5. **Presentation Preparation**
   - Prepare a comprehensive presentation on the developed voting machine and its embedded backdoors.
   - Document the process and results of analyzing and exploiting the other team’s voting machine.
   - Discuss the findings and experiences from both the defensive and offensive perspectives.

## Attribution

The original version of this project was developed for the course Security & Privacy in Computing at Johns Hopkins University. More information can be found in “Root the (Ballot) Box: Designing Security Engineering Courses with E-Voting”, a poster published in the proceedings of the ACM Technical Symposium on Computer Science Education 2024 (SIGCSE ’24).

---

This README provides a detailed outline of the voting machine project, covering development, analysis, exploitation, and presentation phases.
