### Notes

This program runs using three threads.


### Main thread - EGui menu
    - Handles application saving and exiting
    - Updates and saves the config in both physical and virtual memory
### Thread 2 - Game update thread
    - Reads the game's memory and updates generally useful values
    - Exits based on an AtomicBool handled by main thread
### Thread 3 - Overlay thread
    - Hijacks the Nvidia Geforce Overlay
    - Uses D2D for rendering (using windows-rs, fully memory safe!)
    - Exits based on an AtomicBool handled by main thread