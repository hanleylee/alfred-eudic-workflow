<?php

require 'vendor/autoload.php';

use Alfred\Workflows\Workflow;

class Updater
{
    private $currentVersion;
    private $workflow;

    const RELEASE_API = "https://api.github.com/repos/HanleyLee/alfred-eudic-workflow/releases";

    public function __construct($version)
    {
        $this->workflow = new Workflow;
        $this->currentVersion = $version;
    }

    public function fetchReleases()
    {
        $userAgent = 'Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_3) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/79.0.3945.130 Safari/537.36';

        $response = $this->workflow->request(self::RELEASE_API, [CURLOPT_USERAGENT => $userAgent]);
        $this->result = json_decode($response);

        foreach($this->result as $release) {
            $version = $release->tag_name;
            if(version_compare($version, $this->currentVersion) === 1) {
                $releaseAt = 'Released at ' . date('Y-m-d H:i:s', strtotime($release->published_at));;
                $downloadUrl = $release->assets[0]->browser_download_url;
                $this->addItem($version, $releaseAt, $downloadUrl);
            }            
        }

        if(empty($this->workflow->results)) {
            $this->addItem('Already latest version', 'No available updates', null);
        }

        return $this->workflow->output();
    }

    private function addItem($title, $subtitle, $arg)
    {
        $this->workflow->result()
            ->title($title)
            ->subtitle($subtitle)
            ->arg($arg);
    }
}
