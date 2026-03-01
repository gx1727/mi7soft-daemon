import { motion } from 'framer-motion';
import { User } from 'lucide-react';
import { useTranslation, Trans } from 'react-i18next';

const About = () => {
  const { t } = useTranslation();

  const team = [
    { name: "Sarah Chen", role: t('about.team.members.sarah.role'), bio: t('about.team.members.sarah.bio') },
    { name: "James Miller", role: t('about.team.members.james.role'), bio: t('about.team.members.james.bio') },
    { name: "Elena Rodriguez", role: t('about.team.members.elena.role'), bio: t('about.team.members.elena.bio') },
    { name: "David Kim", role: t('about.team.members.david.role'), bio: t('about.team.members.david.bio') }
  ];

  const milestones = [
    { year: "2024 Q1", title: t('about.timeline.milestones.inception.title'), description: t('about.timeline.milestones.inception.description') },
    { year: "2024 Q3", title: t('about.timeline.milestones.alpha.title'), description: t('about.timeline.milestones.alpha.description') },
    { year: "2025 Q1", title: t('about.timeline.milestones.seriesA.title'), description: t('about.timeline.milestones.seriesA.description') },
    { year: "2025 Q2", title: t('about.timeline.milestones.launch.title'), description: t('about.timeline.milestones.launch.description') }
  ];

  return (
    <div className="pt-24 pb-20">
      <div className="container mx-auto px-6">
        {/* Hero Section */}
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          className="max-w-4xl mx-auto text-center mb-24"
        >
          <h1 className="text-4xl md:text-5xl font-display font-bold mb-8">
            <Trans i18nKey="about.hero.title" components={{ 1: <span className="text-gradient" /> }} />
          </h1>
          <p className="text-xl text-gray-400 mb-12 leading-relaxed">
            {t('about.hero.description')}
          </p>
          
          <div className="flex justify-center space-x-12">
            <div className="text-center">
              <div className="text-3xl font-bold text-white mb-2">500+</div>
              <div className="text-sm text-gray-500">{t('about.hero.stats.users')}</div>
            </div>
            <div className="text-center">
              <div className="text-3xl font-bold text-white mb-2">99.99%</div>
              <div className="text-sm text-gray-500">{t('about.hero.stats.uptime')}</div>
            </div>
            <div className="text-center">
              <div className="text-3xl font-bold text-white mb-2">0ms</div>
              <div className="text-sm text-gray-500">{t('about.hero.stats.overhead')}</div>
            </div>
          </div>
        </motion.div>

        {/* Timeline Section */}
        <div className="mb-24">
            <h2 className="text-3xl font-display font-bold text-center mb-16">{t('about.timeline.title')}</h2>
            <div className="relative max-w-4xl mx-auto">
                {/* Vertical Line */}
                <div className="absolute left-1/2 transform -translate-x-1/2 h-full w-px bg-white/10" />
                
                {milestones.map((milestone, index) => (
                    <motion.div 
                        key={index}
                        initial={{ opacity: 0, x: index % 2 === 0 ? -50 : 50 }}
                        whileInView={{ opacity: 1, x: 0 }}
                        viewport={{ once: true }}
                        transition={{ duration: 0.5, delay: index * 0.1 }}
                        className={`relative flex items-center justify-between mb-12 ${index % 2 === 0 ? 'flex-row-reverse' : ''}`}
                    >
                        <div className="w-5/12" />
                        <div className="absolute left-1/2 transform -translate-x-1/2 w-4 h-4 rounded-full bg-accent-cyan border-4 border-background" />
                        <div className={`w-5/12 ${index % 2 === 0 ? 'text-right' : 'text-left'}`}>
                            <div className="bg-surface-card p-6 rounded-xl border border-white/5 hover:border-accent-cyan/30 transition-colors">
                                <span className="text-accent-cyan font-bold text-sm mb-2 block">{milestone.year}</span>
                                <h3 className="text-xl font-bold text-white mb-2">{milestone.title}</h3>
                                <p className="text-gray-400 text-sm">{milestone.description}</p>
                            </div>
                        </div>
                    </motion.div>
                ))}
            </div>
        </div>

        {/* Team Section */}
        <div>
            <h2 className="text-3xl font-display font-bold text-center mb-16">{t('about.team.title')}</h2>
            <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-8">
                {team.map((member, index) => (
                    <motion.div
                        key={index}
                        initial={{ opacity: 0, y: 20 }}
                        whileInView={{ opacity: 1, y: 0 }}
                        viewport={{ once: true }}
                        transition={{ duration: 0.5, delay: index * 0.1 }}
                        className="group bg-surface-card p-6 rounded-xl border border-white/5 hover:border-accent-pink/30 transition-all text-center"
                    >
                        <div className="w-24 h-24 mx-auto bg-white/5 rounded-full mb-6 flex items-center justify-center group-hover:scale-110 transition-transform duration-300">
                            <User className="w-10 h-10 text-gray-400 group-hover:text-accent-pink transition-colors" />
                        </div>
                        <h3 className="text-lg font-bold text-white mb-1">{member.name}</h3>
                        <span className="text-sm text-accent-pink font-medium mb-4 block">{member.role}</span>
                        <p className="text-sm text-gray-400 leading-relaxed">
                            {member.bio}
                        </p>
                    </motion.div>
                ))}
            </div>
        </div>
      </div>
    </div>
  );
};

export default About;
