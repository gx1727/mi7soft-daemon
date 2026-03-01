import { motion } from 'framer-motion';
import { Cpu, RefreshCw, FileJson, Server, Database, Settings, Terminal, FileText } from 'lucide-react';
import CodeBlock from '../components/ui/CodeBlock';
import { useTranslation, Trans } from 'react-i18next';

const Features = () => {
  const { t } = useTranslation();

  const features = [
    {
      id: 'multi-process',
      title: t('features.list.multiProcess.title'),
      description: t('features.list.multiProcess.description'),
      benefits: t('features.list.multiProcess.benefits', { returnObjects: true }),
      icon: <Server className="w-6 h-6 text-accent-cyan" />,
      content: (
        <CodeBlock
          code={`# Swoole 进程架构
Master (PID: 1234, PGID: 1234)
├── Manager
├── Worker 1 (PGID: 1234)
├── Worker 2 (PGID: 1234)
└── TaskWorker (PGID: 1234)

# restart 时自动杀死整个进程组
$ m7d restart swoole-server
> Sending SIGTERM to PGID -1234
> All processes terminated
> Server restarted successfully`}
          language="bash"
        />
      )
    },
    {
      id: 'performance',
      title: t('features.list.performance.title'),
      description: t('features.list.performance.description'),
      benefits: t('features.list.performance.benefits', { returnObjects: true }),
      icon: <Cpu className="w-6 h-6 text-accent-pink" />,
      content: (
        <div className="bg-surface-card p-6 rounded-xl border border-white/5">
          <div className="grid grid-cols-2 gap-6">
            <div className="text-center">
              <div className="text-4xl font-display font-bold text-accent-cyan mb-2">6MB</div>
              <div className="text-sm text-gray-400">Binary Size</div>
            </div>
            <div className="text-center">
              <div className="text-4xl font-display font-bold text-accent-pink mb-2">&lt;10MB</div>
              <div className="text-sm text-gray-400">Memory Usage</div>
            </div>
            <div className="text-center">
              <div className="text-4xl font-display font-bold text-purple-400 mb-2">0</div>
              <div className="text-sm text-gray-400">Runtime Dependencies</div>
            </div>
            <div className="text-center">
              <div className="text-4xl font-display font-bold text-green-400 mb-2">&lt;1s</div>
              <div className="text-sm text-gray-400">Startup Time</div>
            </div>
          </div>
        </div>
      )
    },
    {
      id: 'logging',
      title: t('features.list.logging.title'),
      description: t('features.list.logging.description'),
      benefits: t('features.list.logging.benefits', { returnObjects: true }),
      icon: <Terminal className="w-6 h-6 text-purple-400" />,
      content: (
        <CodeBlock
          code={`# 实时日志跟踪
$ m7d logs my-service --follow
[2026-03-01 11:30:00] [OUT] Server started on port 9501
[2026-03-01 11:30:05] [OUT] Worker 1 ready
[2026-03-01 11:30:05] [ERR] Connection timeout

# 查看最近 1 小时的日志
$ m7d logs my-service --since 3600

# 结构化 JSON 日志
$ RUST_LOG=debug m7d start`}
          language="bash"
        />
      )
    },
    {
      id: 'auto-restart',
      title: t('features.list.autoRestart.title'),
      description: t('features.list.autoRestart.description'),
      benefits: t('features.list.autoRestart.benefits', { returnObjects: true }),
      icon: <RefreshCw className="w-6 h-6 text-green-400" />,
      content: (
        <div className="bg-surface-card p-8 rounded-xl border border-white/5 flex flex-col justify-center h-full space-y-4">
          <div className="flex items-center space-x-4 text-sm">
            <div className="w-2 h-2 rounded-full bg-red-500 animate-pulse" />
            <span className="text-gray-400">Process 'worker-1' exited with code 1</span>
          </div>
          <div className="flex items-center space-x-4 text-sm">
            <div className="w-2 h-2 rounded-full bg-yellow-500" />
            <span className="text-gray-400">Auto-restart enabled, restarting...</span>
          </div>
          <div className="flex items-center space-x-4 text-sm">
            <div className="w-2 h-2 rounded-full bg-green-500" />
            <span className="text-white">Process restarted (PID: 8972)</span>
          </div>
        </div>
      )
    },
    {
      id: 'storage',
      title: t('features.list.storage.title'),
      description: t('features.list.storage.description'),
      benefits: t('features.list.storage.benefits', { returnObjects: true }),
      icon: <Database className="w-6 h-6 text-blue-400" />,
      content: (
        <CodeBlock
          code={`# 查看进程历史
$ m7d history swoole-server
PID 1234 | 2026-03-01 10:00 - 10:30 | 1800s | ✓ Success
PID 5678 | 2026-03-01 09:00 - 09:45 | 2700s | ✗ Failed (code: 1)

# SQLite 数据库位置
~/.local/share/mi7soft-daemon/daemon.db

# 自动清理旧记录
$ m7d cleanup --days 30`}
          language="bash"
        />
      )
    },
    {
      id: 'config',
      title: t('features.list.config.title'),
      description: t('features.list.config.description'),
      benefits: t('features.list.config.benefits', { returnObjects: true }),
      icon: <Settings className="w-6 h-6 text-orange-400" />,
      content: (
        <CodeBlock
          code={`# daemon.toml
[daemon]
pid_file = "/var/run/mi7soft-daemon.pid"
check_interval = 3

[[processes]]
name = "swoole-server"
command = "/usr/bin/php"
args = ["hyperf", "start"]
auto_restart = true
capture_output = true
log_file = "/var/log/swoole.log"

# 热重载配置
$ kill -HUP $(cat /var/run/mi7soft-daemon.pid)`}
          language="toml"
        />
      )
    }
  ];

  return (
    <div className="pt-24 pb-20">
      <div className="container mx-auto px-6">
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          className="text-center mb-20"
        >
          <h1 className="text-4xl md:text-5xl font-display font-bold mb-6">
            <Trans i18nKey="features.title" components={{ 1: <span className="text-gradient" /> }} />
          </h1>
          <p className="text-xl text-gray-400 max-w-2xl mx-auto">
            {t('features.subtitle')}
          </p>
        </motion.div>

        <div className="space-y-32">
          {features.map((feature, index) => (
            <motion.div
              key={feature.id}
              initial={{ opacity: 0, y: 40 }}
              whileInView={{ opacity: 1, y: 0 }}
              viewport={{ once: true, margin: "-100px" }}
              transition={{ duration: 0.6 }}
              className={`flex flex-col ${index % 2 === 1 ? 'md:flex-row-reverse' : 'md:flex-row'} items-center gap-12 md:gap-20`}
            >
              <div className="w-full md:w-1/2">
                <div className="flex items-center space-x-4 mb-6">
                  <div className="p-3 bg-white/5 rounded-lg border border-white/10">
                    {feature.icon}
                  </div>
                  <h2 className="text-3xl font-display font-bold">{feature.title}</h2>
                </div>
                <p className="text-lg text-gray-400 leading-relaxed mb-8">
                  {feature.description}
                </p>
                {Array.isArray(feature.benefits) && (
                  <ul className="space-y-3">
                    {feature.benefits.map((benefit: string, i: number) => (
                      <li key={i} className="flex items-center text-gray-300">
                        <div className="w-1.5 h-1.5 rounded-full bg-accent-cyan mr-3" />
                        {benefit}
                      </li>
                    ))}
                  </ul>
                )}
              </div>
              <div className="w-full md:w-1/2">
                <div className="relative group">
                  <div className="absolute -inset-1 bg-gradient-to-r from-accent-cyan to-accent-pink rounded-xl opacity-20 blur group-hover:opacity-40 transition duration-1000"></div>
                  <div className="relative">
                    {feature.content}
                  </div>
                </div>
              </div>
            </motion.div>
          ))}
        </div>

        {/* CTA Section */}
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          whileInView={{ opacity: 1, y: 0 }}
          viewport={{ once: true }}
          className="mt-32 text-center"
        >
          <h2 className="text-3xl md:text-4xl font-display font-bold mb-6">
            Ready to Get Started?
          </h2>
          <p className="text-xl text-gray-400 mb-8 max-w-2xl mx-auto">
            Download mi7soft-daemon and start managing your processes efficiently.
          </p>
          <div className="flex flex-col sm:flex-row items-center justify-center space-y-4 sm:space-y-0 sm:space-x-6">
            <a
              href="https://github.com/gx1727/mi7soft-daemon/releases/latest"
              target="_blank"
              rel="noopener noreferrer"
              className="px-8 py-4 bg-accent-cyan text-black font-bold rounded-lg hover:bg-accent-cyan/90 transition-all"
            >
              Download Latest Version
            </a>
            <a
              href="https://github.com/gx1727/mi7soft-daemon#readme"
              target="_blank"
              rel="noopener noreferrer"
              className="px-8 py-4 bg-white/5 border border-white/10 text-white font-bold rounded-lg hover:bg-white/10 transition-all backdrop-blur-sm flex items-center"
            >
              <FileText className="mr-2 w-5 h-5" />
              Read Documentation
            </a>
          </div>
        </motion.div>
      </div>
    </div>
  );
};

export default Features;
