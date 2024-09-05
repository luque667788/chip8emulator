import init, { run } from "./pkg/emulatorwasm2.js";

        async function start() {
            let canvas = null;

            const fileInput = document.getElementById('rom-upload');
            const uploadButton = document.getElementById('upload-button');
            const fileNameDisplay = document.getElementById('file-name');

            const uploadSection = document.querySelector('.upload-section');
            //const creditSection = document.querySelector('.credit-section');
            const keyboardLayout = document.getElementById("keyboard-layout");
            const title = document.getElementById("title");
            const roms = [
                    "TEST ROM.ch8",
                    "IBM Logo.ch8",
                    "PONG",
                    "TETRIS",
                    "BLITZ",
                    "BRIX",
                    "CONNECT4",
                    "GUESS",
                    "HIDDEN",
                    "INVADERS",
                    "MISSILE",                    
                    "PONG2",
                    "TANK",
                    "TICTAC",
                    "UFO",
                    "VBRIX",
                    "WIPEOFF"
                ];

            const romSelect = document.getElementById("rom-select");

          roms.forEach((rom) => {
            const option = document.createElement("option");
            option.value = rom;
            option.textContent = rom;
            romSelect.appendChild(option);
          });

          romSelect.addEventListener("change", async (event) => {
            const selectedRom = event.target.value;
            if (selectedRom) {
              const response = await fetch(`https://luque667788.github.io/chip8emulator/roms/${selectedRom}`);
              const romData = await response.arrayBuffer();
              const romDataArray = new Uint8Array(romData);
              // Load the ROM data into the emulator
              // Save the ROM data to localStorage
            localStorage.setItem("latestRomData", JSON.stringify(Array.from(romDataArray)));
            localStorage.setItem("latestRomName", selectedRom);
            localStorage.removeItem('uploadedFile');
            localStorage.removeItem('uploadedFileName');
            location.reload();
            }
          });
            
            
            uploadButton.addEventListener('click', () => {
                fileInput.click();
                console.log("food is good");
            });

            fileInput.addEventListener('change', async (event) => {
                const file = event.target.files[0];
                if (file) {
                    fileNameDisplay.textContent = file.name;
                    const arrayBuffer = await file.arrayBuffer();
                    const uint8Array = new Uint8Array(arrayBuffer);
                    localStorage.setItem('uploadedFile', JSON.stringify(Array.from(uint8Array)));
                    localStorage.setItem('uploadedFileName', file.name);
                    localStorage.removeItem('latestRomData');
                    localStorage.removeItem('latestRomName');
                    location.reload();
                }
            });
            function getTextDimensions(element) {
                const style = window.getComputedStyle(element);
                const font = `${style.fontWeight} ${style.fontSize} ${style.fontFamily}`;
                const canvas = document.createElement('canvas');
                const context = canvas.getContext('2d');
                context.font = font;
                const metrics = context.measureText(element.textContent);
                
                const elementRect = element.getBoundingClientRect();
                const textWidth = metrics.width;
                const textLeft = elementRect.left + (elementRect.width - textWidth) / 2;
                
                return {
                    left: textLeft,
                    top: elementRect.top,
                    right: textLeft + textWidth,
                    bottom: elementRect.bottom,
                    width: textWidth,
                    height: elementRect.height
                };
            }


            function checkIntersection() {
                const keyboardRect = keyboardLayout.getBoundingClientRect();
                const uploadRect = uploadSection.getBoundingClientRect();
                //const creditRect = creditSection.getBoundingClientRect();
                const titleRect = getTextDimensions(title);

                let shouldHide = intersects(keyboardRect, uploadRect)  || intersects(keyboardRect,titleRect);

                // Check canvas intersection if it exists
                if (canvas) {
                    const canvasRect = canvas.getBoundingClientRect();
                    shouldHide = shouldHide || intersects(keyboardRect, canvasRect);
                }

                keyboardLayout.style.opacity = shouldHide ? '0' : '1';
            }

            function intersects(rect1, rect2) {
                return !(rect1.right < rect2.left || 
                        rect1.left > rect2.right || 
                        rect1.bottom < rect2.top || 
                        rect1.top > rect2.bottom);
            }
            checkIntersection();
            window.addEventListener('scroll', checkIntersection);
            window.addEventListener('resize', checkIntersection);

            // Focus the canvas element once it is created
            const observer = new MutationObserver((mutations) => {
                mutations.forEach((mutation) => {
                    mutation.addedNodes.forEach((node) => {
                        if (node.tagName === 'CANVAS') {
                            node.focus();
                            //observer2.observe(node);
                            canvas = node;
                            observer.disconnect(); // Stop observing once the canvas is found and focused
                        }
                    });
                });
            });


            observer.observe(document.body, { childList: true, subtree: true });

            // Check if there's a file stored in localStorage
            const storedFile = localStorage.getItem('uploadedFile');
            // Load the latest ROM data from localStorage if available
            const savedRomData = localStorage.getItem("latestRomData");
            const savedRomName = localStorage.getItem("latestRomName");
            
            if (storedFile) {
                const fileName = localStorage.getItem('uploadedFileName');
                fileNameDisplay.textContent = fileName;
                const uint8Array = new Uint8Array(JSON.parse(storedFile));
                await init();
                console.log("WASM Loaded");
                console.log("uploading from client");
                await run(uint8Array);
                
            }
            else if (savedRomData && savedRomName) {
                await init();
                console.log("WASM Loaded");
                const romDataArray = new Uint8Array(JSON.parse(savedRomData));
                console.log("uploading from server");
                await run(romDataArray);                
            }
        }

        start();