var panX = 2;
var panY = 0.8;
var magFactor = 500;
var maxIterations = 25;
var hue = 30;
var canvas = document.getElementById("mandelbrotCanvas");
var context = canvas.getContext("2d");
context.canvas.width = window.innerWidth;
context.canvas.height = window.innerHeight;

function checkMandelBrotSet(x,y){
  var tempx = x;
  var tempy = y;
  var currx;
  var curry;
  for (var i = 0; i < maxIterations; i++){
    currx = tempx * tempx - tempy * tempy + x;
    curry =  2 * tempx * tempy + y;

    tempx = currx;
    tempy = curry;

    if(currx *  curry > 5)
      return (i / maxIterations * 100);
  }
  return 0;
}

function Generate(){
  for(var x = 0 ; x < context.canvas.width; x++){
    for(var y = 0; y < context.canvas.height; y++){
      var belongsToSet = checkMandelBrotSet(x / magFactor - panX , y / magFactor - panY);

      if(belongsToSet === 0){
        context.fillStyle = "#000000";
        context.fillRect(x,y,1,1);
      }
      else{
        context.fillStyle = `hsl(${hue}, 100%, ${belongsToSet}%)`;
        context.fillRect(x,y,1,1);
      }
    }
  }
}


function PanXVal(value){
  panX = value;
}

function PanYVal(value){
  panY = value;
}

function Val(value){
  magFactor = value;
}

function HueVal(value){
  hue = value;
}

function ItVal(value){
  maxIterations = value;
}












