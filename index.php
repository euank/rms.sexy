<?php
$img = array();
$handler = opendir('img/');

while ($file = readdir($handler)) {
  if ($file != "." && $file != "..") {
    $img[] = $file;
  }
}

closedir($handler);

$birthday = (new DateTime("now", new DateTimeZone('America/Los_Angeles')))->format('m-d') == '03-16';
$birthday = $birthday || (new DateTime("now", new DateTimeZone('Australia/Sydney')))->format('m-d') == '03-16';
?>
<!DOCTYPE html>
<html>
<head>
   <?php if(!$birthday) { ?>
   <title>Our GNU/Lord and GNU/Savior is 100% sexy!</title>
   <?php } else { ?>
   <title>Happy GNU/Birthday!</title>
   <?php } ?>
   <meta http-equiv="refresh" content="3;/"></meta>
   <link rel="stylesheet" href="/style.css">
</head>
<body>
   <a href="https://donate.fsf.org/"><img class="donate" src="https://rms.sexy/donate.png" alt="Donate!" title="Donate to the FSF!"></a>
   <a href="/"><img class="stallman" src="/img/<?=$img[array_rand($img)];?>" height="100%"></a>
</body>
</html>
