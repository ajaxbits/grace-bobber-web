// fix the video height on android screens
let vh = window.innerHeight * 0.01;
document.documentElement.style.setProperty("--vh", `${vh}px`);
// We listen to the resize event
window.addEventListener("resize", () => {
  // We execute the same script as before
  let vh = window.innerHeight * 0.01;
  document.documentElement.style.setProperty("--vh", `${vh}px`);
});

// handle mute button functionality
const muteButton = document.querySelector(`.mutebutton`) as HTMLElement;
const video = document.querySelector(`.video`) as any;

const handleClick = (e) => {
  video.muted ? (video.muted = false) : (video.muted = true);
  muteButton.style.removeProperty(`animation`);
};

muteButton.addEventListener(`click`, handleClick);
