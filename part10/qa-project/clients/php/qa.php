<?php
require dirname(__FILE__) . '/vendor/autoload.php';

const GRPC_ADDRESS = '127.0.0.1:50051';

// php qa.php daheige 123456
function user_login($name,$pwd)
{
    $client = new App\Grpc\Qa\QAServiceClient(GRPC_ADDRESS, [
        'credentials' => Grpc\ChannelCredentials::createInsecure(),
    ]);

    $request = new App\Grpc\Qa\UserLoginRequest();
    $request->setUsername($name);
    $request->setPassword($pwd);
    list($reply, $status) = $client->UserLogin($request)->wait();
    echo 'status code: ' . $status->code;
    if ($status->details) {
        echo ', details: ' . $status->details;
    }

    echo PHP_EOL;
    if ($status->metadata) {
        echo 'Meta data: ' . PHP_EOL;
        print_r($status->metadata);
    }

    echo "reply token: ".$reply->getToken();
    echo PHP_EOL;
}

$name = !empty($argv[1]) ? $argv[1] : 'daheige';
$pwd = !empty($argv[2]) ? $argv[2] : '123456';
user_login($name,$pwd);
