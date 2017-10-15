<?php
$imgdir = 'img';
$img = array();
$handler = opendir($imgdir);

while ($file = readdir($handler)) {
	if ($file != "." && $file != "..") {
		$img[] = "/$imgdir/$file";
	}
}

closedir($handler);

if (isset($_GET['images'])) {
	header('Content-Type: application/json');
	die(json_encode($img));
}

$birthday = (new DateTime("now", new DateTimeZone('America/Los_Angeles')))->format('m-d') == '03-16';
$birthday = $birthday || (new DateTime("now", new DateTimeZone('Australia/Sydney')))->format('m-d') == '03-16';
?>
<!DOCTYPE html>
<html>
	<head>
		<!--
This is free and unencumbered software released into the public domain.

Anyone is free to copy, modify, publish, use, compile, sell, or
distribute this software, either in source code form or as a compiled
binary, for any purpose, commercial or non-commercial, and by any
means.

The source code, along with the full license text, is available here:
https://github.com/Mechazawa/rms.sexy
		-->
		<meta charset="utf-8">
		<?php if(!$birthday) { ?>
		<title>Our GNU/Lord and GNU/Savior is 100% sexy!</title>
		<?php } else { ?>
		<title>Happy GNU/Birthday!</title>
		<?php } ?>
		<?php if(!isset($_GET['js'])) { ?>
		<meta http-equiv="refresh" content="3;/">
		<?php } ?>
		<link rel="stylesheet" href="/style.css">
		<link rel="license" href="/license.txt">
		<script async src="/script.js"></script>
	</head>
	<body>
		<a href="https://donate.fsf.org/"><img class="donate" src="/donate.png" alt="Donate!" title="Donate to the FSF!"></a>
		<a href="/"><img alt="RMS Matthew Stallman" class="stallman" src="<?=$img[array_rand($img)];?>" height="100%"></a>
	</body>
</html>
