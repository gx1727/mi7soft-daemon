import { motion } from 'framer-motion';
import { Cpu, RefreshCw, FileJson, Server } from 'lucide-react';
import CodeBlock from '../components/ui/CodeBlock';
import PerformanceChart from '../components/ui/PerformanceChart';
import { useTranslation, Trans } from 'react-i18next';

const Features = () => {
  const { t } = useTranslation();

  const features = [
    {
      id: 'multi-process',
      title: t('features.list.multiProcess.title'),
      description: t('features.list.multiProcess.description'),
      icon: <Server className="w-6 h-6 text-accent-cyan" />,
      content: (
        <CodeBlock
          code={`[processes.api]
command = "./api-server"
instances = 4
restart_policy = "always"

[processes.worker]
command = "./background-worker"
instances = 2
depends_on = ["api"]`}
          language="toml"
        />
      )
    },
    {
      id: 'performance',
      title: t('features.list.performance.title'),
      description: t('features.list.performance.description'),
      icon: <Cpu className="w-6 h-6 text-accent-pink" />,
      content: (
        <div className="bg-surface-card p-6 rounded-xl border border-white/5">
          <PerformanceChart />
        </div>
      )
    },
    {
      id: 'hot-reload',
      title: t('features.list.hotReload.title'),
      description: t('features.list.hotReload.description'),
      icon: <FileJson className="w-6 h-6 text-purple-400" />,
      content: (
        <CodeBlock
          code={`$ mi7 reload
> Configuration changes detected
> Diff: + [processes.cache]
> Spawning 2 new instances of 'cache'
> Gracefully reloading 'api'
> Done in 45ms`}
          language="bash"
        />
      )
    },
    {
      id: 'auto-restart',
      title: t('features.list.autoRestart.title'),
      description: t('features.list.autoRestart.description'),
      icon: <RefreshCw className="w-6 h-6 text-green-400" />,
      content: (
        <div className="bg-surface-card p-8 rounded-xl border border-white/5 flex flex-col justify-center h-full space-y-4">
            <div className="flex items-center space-x-4 text-sm">
                <div className="w-2 h-2 rounded-full bg-red-500 animate-pulse" />
                <span className="text-gray-400">Process 'worker-1' exited with code 1</span>
            </div>
            <div className="flex items-center space-x-4 text-sm">
                <div className="w-2 h-2 rounded-full bg-yellow-500" />
                <span className="text-gray-400">Waiting 1000ms (exponential backoff)...</span>
            </div>
            <div className="flex items-center space-x-4 text-sm">
                <div className="w-2 h-2 rounded-full bg-green-500" />
                <span className="text-white">Restarting 'worker-1'... Success (PID: 8972)</span>
            </div>
        </div>
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
                <ul className="space-y-3">
                    {[1, 2, 3].map((i) => (
                        <li key={i} className="flex items-center text-gray-300">
                            <div className="w-1.5 h-1.5 rounded-full bg-accent-cyan mr-3" />
                            Feature benefit point {i}
                        </li>
                    ))}
                </ul>
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
      </div>
    </div>
  );
};

export default Features;
