<?php
function getDirectoryList ($directory) {
    $results = array();
    $handler = opendir($directory);
    while ($file = readdir($handler)) {
      if ($file != "." && $file != "..") {
        $results[] = $file;
      }
    }
    closedir($handler);
    return $results;
  }
$d=getDirectoryList('img/');
?><html>
<head>
   <title>Our GNU/Lord and GNU/Savior is 100% sexy!</title>
   <meta http-equiv="refresh" content="3;https://rms.sexy/">	
   <!--
       I'm too lazy to make the html pretty for now... Maybe when I've drank
       enough booze and I have nothing to do... Or if people start nagging  
       about it.
   -->
   <style>
  body {
    margin: 0;
    background-image: url("/noise.png");
  }

  img {
    text-align: center;
    position: absolute;
    margin: auto;
    top: 0;
    right: 0;
    bottom: 0;
    left: 0;
image-orientation: from-image;
  }

    </style>
</head>
<body>
   <a href="/"><img class="stallman" src="/img/<?=$d[array_rand($d)];?>" height="100%"></a>
</body>
</html>
