import { motion } from 'framer-motion';
import { Github, Mail } from 'lucide-react';
import { useTranslation, Trans } from 'react-i18next';

const About = () => {
  const { t } = useTranslation();

  return (
    <div className="pt-24 pb-20">
      <div className="container mx-auto px-6 max-w-4xl">
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          className="text-center mb-16"
        >
          <h1 className="text-4xl md:text-5xl font-display font-bold mb-6">
            <Trans i18nKey="about.hero.title" components={{ 1: <span className="text-gradient" /> }} />
          </h1>
          <p className="text-xl text-gray-400 leading-relaxed">
            {t('about.hero.description')}
          </p>
        </motion.div>

        {/* Project Stats */}
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ delay: 0.1 }}
          className="grid grid-cols-1 md:grid-cols-3 gap-8 mb-16"
        >
          <div className="bg-surface-card p-6 rounded-xl border border-white/5 text-center">
            <div className="text-3xl font-display font-bold text-accent-cyan mb-2">v0.1.2</div>
            <div className="text-gray-400">Latest Version</div>
          </div>
          <div className="bg-surface-card p-6 rounded-xl border border-white/5 text-center">
            <div className="text-3xl font-display font-bold text-accent-pink mb-2">6MB</div>
            <div className="text-gray-400">Binary Size</div>
          </div>
          <div className="bg-surface-card p-6 rounded-xl border border-white/5 text-center">
            <div className="text-3xl font-display font-bold text-purple-400 mb-2">Rust</div>
            <div className="text-gray-400">Built With</div>
          </div>
        </motion.div>

        {/* Open Source */}
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ delay: 0.2 }}
          className="bg-surface-card p-8 rounded-xl border border-white/5 mb-16"
        >
          <h2 className="text-2xl font-display font-bold mb-4">Open Source Project</h2>
          <p className="text-gray-400 leading-relaxed mb-6">
            mi7soft-daemon is an open-source project focused on solving process management challenges 
            for Swoole, Hyperf, and microservices. It's built by developers, for developers.
          </p>
          <div className="flex flex-col sm:flex-row gap-4">
            <a
              href="https://github.com/gx1727/mi7soft-daemon"
              target="_blank"
              rel="noopener noreferrer"
              className="flex items-center justify-center px-6 py-3 bg-white/5 border border-white/10 rounded-lg hover:bg-white/10 transition-colors"
            >
              <Github className="w-5 h-5 mr-2" />
              View on GitHub
            </a>
            <a
              href="https://github.com/gx1727/mi7soft-daemon/issues"
              target="_blank"
              rel="noopener noreferrer"
              className="flex items-center justify-center px-6 py-3 bg-white/5 border border-white/10 rounded-lg hover:bg-white/10 transition-colors"
            >
              <Mail className="w-5 h-5 mr-2" />
              Report Issue
            </a>
          </div>
        </motion.div>

        {/* License */}
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ delay: 0.3 }}
          className="text-center"
        >
          <p className="text-gray-400">
            Released under the <span className="text-accent-cyan">MIT License</span>
          </p>
          <p className="text-sm text-gray-500 mt-2">
            Free to use, modify, and distribute
          </p>
        </motion.div>
      </div>
    </div>
  );
};

export default About;
