const hamburger=document.querySelector(".hamburger"),menu=document.querySelector(".menu"),closeButton=document.querySelector(".close"),toggleMenu=()=>{hamburger&&menu&&closeButton&&menu.classList.toggle("open"),document.querySelector("body").classList.toggle("disable-scroll")};hamburger.addEventListener("click",toggleMenu),closeButton.addEventListener("click",toggleMenu);