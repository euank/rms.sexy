<?php
$img = array();
$handler = opendir('img/');

while ($file = readdir($handler)) {
  if ($file != "." && $file != "..") {
    $img[] = $file;
  }
}

closedir($handler);
?>
<!DOCTYPE html>
<html>
<head>
   <title>Our GNU/Lord and GNU/Savior is 100% sexy!</title>
   <meta http-equiv="refresh" content="3;/"></meta>
   <link rel="stylesheet" href="/style.css">
</head>
<body>
   <a href="https://donate.fsf.org/"><img class="donate" src="https://rms.sexy/donate.png" alt="Donate!" title="Donate to the FSF!"></a>
   <a href="/"><img class="stallman" src="/img/<?=$img[array_rand($img)];?>" height="100%"></a>
</body>
</html>
