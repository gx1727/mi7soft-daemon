import { motion } from 'framer-motion';
import { Github, Mail, MessageCircle } from 'lucide-react';
import { useTranslation } from 'react-i18next';

const Contact = () => {
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
            {t('contact.title')}
          </h1>
          <p className="text-xl text-gray-400">
            {t('contact.subtitle')}
          </p>
        </motion.div>

        {/* Contact Methods */}
        <div className="grid grid-cols-1 md:grid-cols-3 gap-8">
          <motion.a
            href="https://github.com/gx1727/mi7soft-daemon/issues"
            target="_blank"
            rel="noopener noreferrer"
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ delay: 0.1 }}
            className="bg-surface-card p-8 rounded-xl border border-white/5 hover:border-white/10 transition-colors group"
          >
            <div className="flex flex-col items-center text-center">
              <div className="p-4 bg-white/5 rounded-xl mb-4 group-hover:bg-white/10 transition-colors">
                <Github className="w-8 h-8 text-accent-cyan" />
              </div>
              <h3 className="text-xl font-display font-bold mb-2">GitHub Issues</h3>
              <p className="text-gray-400">Bug reports & feature requests</p>
            </div>
          </motion.a>

          <motion.a
            href="https://github.com/gx1727/mi7soft-daemon/discussions"
            target="_blank"
            rel="noopener noreferrer"
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ delay: 0.2 }}
            className="bg-surface-card p-8 rounded-xl border border-white/5 hover:border-white/10 transition-colors group"
          >
            <div className="flex flex-col items-center text-center">
              <div className="p-4 bg-white/5 rounded-xl mb-4 group-hover:bg-white/10 transition-colors">
                <MessageCircle className="w-8 h-8 text-accent-pink" />
              </div>
              <h3 className="text-xl font-display font-bold mb-2">Discussions</h3>
              <p className="text-gray-400">Community support & questions</p>
            </div>
          </motion.a>

          <motion.div
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ delay: 0.3 }}
            className="bg-surface-card p-8 rounded-xl border border-white/5"
          >
            <div className="flex flex-col items-center text-center">
              <div className="p-4 bg-white/5 rounded-xl mb-4">
                <Mail className="w-8 h-8 text-purple-400" />
              </div>
              <h3 className="text-xl font-display font-bold mb-2">Email</h3>
              <p className="text-gray-400 text-sm">Check GitHub profile</p>
            </div>
          </motion.div>
        </div>

        {/* Contribute */}
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ delay: 0.4 }}
          className="mt-16 text-center"
        >
          <p className="text-gray-400 mb-4">
            Want to contribute? Pull requests are welcome!
          </p>
          <a
            href="https://github.com/gx1727/mi7soft-daemon/pulls"
            target="_blank"
            rel="noopener noreferrer"
            className="text-accent-cyan hover:text-accent-cyan/80 transition-colors"
          >
            View open pull requests →
          </a>
        </motion.div>
      </div>
    </div>
  );
};

export default Contact;
