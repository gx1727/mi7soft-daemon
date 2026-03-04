<?php
/**
 * Test script for mi7soft-daemon
 * 模拟一个长时间运行的进程
 */

// 记录启动时间
$startTime = time();
$pid = getmypid();

echo "[PID:{$pid}] Test process started at " . date('Y-m-d H:i:s') . "\n";

// 模拟配置
$configFile = __DIR__ . '/test-daemon-config.toml';

// 检查命令参数
$command = $argv[1] ?? 'run';

switch ($command) {
    case 'run':
        // 正常模式：持续运行
        $counter = 0;
        while (true) {
            $counter++;
            echo "[PID:{$pid}] Running... counter={$counter} time=" . date('H:i:s') . "\n";
            
            // 每 5 秒输出一次
            sleep(5);
            
            // 模拟偶尔的错误
            if ($counter % 20 === 0) {
                fwrite(STDERR, "[PID:{$pid}] Warning: simulated warning\n");
            }
        }
        break;
        
    case 'once':
        // 单次模式：运行一次后退出
        echo "[PID:{$pid}] Running once task...\n";
        sleep(2);
        echo "[PID:{$pid}] Task completed\n";
        exit(0);
        break;
        
    case 'fail':
        // 失败模式：模拟崩溃
        echo "[PID:{$pid}] Simulating failure...\n";
        sleep(1);
        exit(1);
        break;
        
    case 'daemon':
        // 守护模式：fork 后在后台运行
        echo "[PID:{$pid}] Starting as daemon...\n";
        
        $childPid = pcntl_fork();
        if ($childPid == -1) {
            die("fork failed");
        } elseif ($childPid > 0) {
            // 父进程退出
            echo "[PID:{$pid}] Parent exiting, child PID: {$childPid}\n";
            exit(0);
        }
        
        // 子进程成为新会话的 leader
        posix_setsid();
        
        // 重新输出
        $newPid = getmypid();
        echo "[PID:{$newPid}] Daemon started\n";
        
        file_put_contents(__DIR__ . '/test-daemon.pid', $newPid);
        
        $counter = 0;
        while (true) {
            $counter++;
            echo "[PID:{$newPid}] Daemon running... {$counter}\n";
            sleep(3);
        }
        break;
        
    case 'status':
        // 状态模式：输出状态信息
        $uptime = time() - $startTime;
        echo "Status: RUNNING\n";
        echo "PID: {$pid}\n";
        echo "Uptime: {$uptime} seconds\n";
        echo "Memory: " . memory_get_usage(true) . " bytes\n";
        break;
        
    case 'config':
        // 输出测试配置
        $config = <<<CONFIG
[daemon]
pid_file = "/var/run/mi7soft-test.pid"
log_file = "/var/log/mi7soft-test.log"
check_interval = 3

[[processes]]
name = "test-run"
command = "php"
args = ["test.php", "run"]
working_directory = "/root/work/mi7soft-daemon"
auto_restart = true
capture_output = true
log_file = "/var/log/mi7soft-test-run.log"
max_instances = 1

[[processes]]
name = "test-once"
command = "php"
args = ["test.php", "once"]
working_directory = "/root/work/mi7soft-daemon"
auto_restart = true
capture_output = true
log_file = "/var/log/mi7soft-test-once.log"

[[processes]]
name = "test-fail"
command = "php"
args = ["test.php", "fail"]
working_directory = "/root/work/mi7soft-daemon"
auto_restart = true
capture_output = true
log_file = "/var/log/mi7soft-test-fail.log"
CONFIG;
        echo $config;
        break;
        
    default:
        echo "Usage: php test.php [run|once|fail|daemon|status|config]\n";
        exit(1);
}
