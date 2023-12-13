const sendButton = document.querySelector(`.sendButton`);
const contactForm = document.querySelector(`form`);
const formModal = document.querySelector(`formModal`);

const handleSubmit = () => {
  console.log(`works!`);
};

const handleCloseButton = (e) => {
  if (!e.target.matches(`.formModal-close`)) return;

  formModal.classList.remove(`formModal-open`);
};

document.addEventListener(`click`, (e) => {
  handleCloseButton(e);
});

contactForm.addEventListener(`submit`, handleSubmit);
