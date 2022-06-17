// sticky, but not annoying nav using headroom.js
import Headroom from "./headroom.js";
const siteNav = document.querySelector(`nav`) as HTMLElement;
const videoWrapper = document.querySelector(`.videoWrapper`) as HTMLElement;
let options;
videoWrapper
  ? (options = { offset: videoWrapper.clientHeight })
  : (options = {});
var headroom = new Headroom(siteNav, options);
headroom.init();

// activate the swipeover actions
const handleSwipeOver = (e) => {
  const navHamburger: boolean =
    e.target.matches(`.siteNav-hamburger`) ||
    e.target.parentElement.matches(`.siteNav-hamburger`);
  const splashHamburger: boolean =
    e.target.matches(`.mobileMenuButton`) ||
    e.target.matches(`.mobileMenuButton-icon`) ||
    e.target.parentElement.matches(`.mobileMenuButton-icon`);
  const bioLink: boolean = e.target.matches(`.bio-link`);
  const closeButton: boolean = e.target.matches(`.close`);

  if (navHamburger || bioLink || closeButton || splashHamburger) {
    const swipeOver = document.querySelector(`.swipeOver`);
    swipeOver.classList.toggle(`swipeOver-isOpen`);
  }
};

const handleSwipeOverDropdown = (e) => {
  const swipeOverDropDowns = document.querySelectorAll(`.swipeOver-dropdown`);

  // make sure it has a nextElementSibling
  if (e.target.nextElementSibling) {
    // delegate event
    if (!e.target.nextElementSibling.matches(`.swipeOver-dropdown`)) return;
    //execute code
    e.preventDefault();
    swipeOverDropDowns.forEach((element) =>
      element.classList.remove(`swipeOver-dropdown-isOpen`)
    );
    e.target.nextElementSibling.classList.add(`swipeOver-dropdown-isOpen`);
  }
};

document.addEventListener(`click`, (e) => {
  handleSwipeOver(e);
  handleSwipeOverDropdown(e);
});
