import { useState } from 'react';
import { motion } from 'framer-motion';
import { useForm } from 'react-hook-form';
import { Mail, MapPin, Phone, Send, CheckCircle, AlertCircle } from 'lucide-react';
import { useTranslation } from 'react-i18next';

type FormData = {
  name: string;
  email: string;
  subject: string;
  message: string;
};

const Contact = () => {
  const { register, handleSubmit, reset, formState: { errors, isSubmitting } } = useForm<FormData>();
  const [submitStatus, setSubmitStatus] = useState<'idle' | 'success' | 'error'>('idle');
  const { t } = useTranslation();

  const onSubmit = async (data: FormData) => {
    try {
      // Simulating API call
      await new Promise(resolve => setTimeout(resolve, 1500));
      
      console.log('Form Submitted:', data);
      setSubmitStatus('success');
      reset();
      
      // Reset status after 5 seconds
      setTimeout(() => setSubmitStatus('idle'), 5000);
    } catch (error) {
      console.error('Error submitting form:', error);
      setSubmitStatus('error');
    }
  };

  return (
    <div className="pt-24 pb-20">
      <div className="container mx-auto px-6">
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          className="text-center mb-16"
        >
          <h1 className="text-4xl md:text-5xl font-display font-bold mb-6">{t('contact.title')}</h1>
          <p className="text-xl text-gray-400 max-w-2xl mx-auto">
            {t('contact.subtitle')}
          </p>
        </motion.div>

        <div className="grid grid-cols-1 lg:grid-cols-2 gap-12 max-w-6xl mx-auto">
          {/* Contact Information */}
          <motion.div
            initial={{ opacity: 0, x: -50 }}
            animate={{ opacity: 1, x: 0 }}
            transition={{ delay: 0.2 }}
            className="space-y-8"
          >
            <div className="bg-surface-card p-8 rounded-2xl border border-white/5">
              <h3 className="text-2xl font-bold mb-6">{t('contact.info.title')}</h3>
              <div className="space-y-6">
                <div className="flex items-start space-x-4">
                  <MapPin className="w-6 h-6 text-accent-cyan mt-1" />
                  <div>
                    <h4 className="font-bold text-white">{t('contact.info.headquarters')}</h4>
                    <p className="text-gray-400">123 Tech Boulevard<br />San Francisco, CA 94107</p>
                  </div>
                </div>
                <div className="flex items-start space-x-4">
                  <Mail className="w-6 h-6 text-accent-cyan mt-1" />
                  <div>
                    <h4 className="font-bold text-white">{t('contact.info.email')}</h4>
                    <p className="text-gray-400">contact@mi7soft.com</p>
                    <p className="text-gray-400">support@mi7soft.com</p>
                  </div>
                </div>
                <div className="flex items-start space-x-4">
                  <Phone className="w-6 h-6 text-accent-cyan mt-1" />
                  <div>
                    <h4 className="font-bold text-white">{t('contact.info.phone')}</h4>
                    <p className="text-gray-400">+1 (555) 123-4567</p>
                  </div>
                </div>
              </div>
            </div>

            <div className="bg-surface-card p-8 rounded-2xl border border-white/5 h-64 relative overflow-hidden group">
                {/* Placeholder Map */}
                <div className="absolute inset-0 bg-[#242f3e] flex items-center justify-center">
                    <span className="text-gray-500">Interactive Map Component</span>
                </div>
                <div className="absolute inset-0 bg-accent-cyan/5 group-hover:bg-accent-cyan/10 transition-colors pointer-events-none" />
            </div>
          </motion.div>

          {/* Contact Form */}
          <motion.div
            initial={{ opacity: 0, x: 50 }}
            animate={{ opacity: 1, x: 0 }}
            transition={{ delay: 0.4 }}
          >
            <div className="bg-surface-card p-8 rounded-2xl border border-white/5">
              <form onSubmit={handleSubmit(onSubmit)} className="space-y-6">
                <div>
                  <label className="block text-sm font-medium text-gray-400 mb-2">{t('contact.form.name')}</label>
                  <input
                    {...register('name', { required: 'Name is required' })}
                    type="text"
                    className="w-full px-4 py-3 bg-background border border-white/10 rounded-lg focus:border-accent-cyan focus:outline-none text-white transition-colors focus:bg-white/5"
                    placeholder={t('contact.form.namePlaceholder')}
                  />
                  {errors.name && <span className="text-red-500 text-xs mt-1">{errors.name.message}</span>}
                </div>
                
                <div>
                  <label className="block text-sm font-medium text-gray-400 mb-2">{t('contact.form.email')}</label>
                  <input
                    {...register('email', { 
                      required: 'Email is required',
                      pattern: {
                        value: /^[A-Z0-9._%+-]+@[A-Z0-9.-]+\.[A-Z]{2,}$/i,
                        message: "Invalid email address"
                      }
                    })}
                    type="email"
                    className="w-full px-4 py-3 bg-background border border-white/10 rounded-lg focus:border-accent-cyan focus:outline-none text-white transition-colors focus:bg-white/5"
                    placeholder={t('contact.form.emailPlaceholder')}
                  />
                  {errors.email && <span className="text-red-500 text-xs mt-1">{errors.email.message}</span>}
                </div>

                <div>
                  <label className="block text-sm font-medium text-gray-400 mb-2">{t('contact.form.subject')}</label>
                  <select
                    {...register('subject', { required: 'Subject is required' })}
                    className="w-full px-4 py-3 bg-background border border-white/10 rounded-lg focus:border-accent-cyan focus:outline-none text-white transition-colors focus:bg-white/5"
                  >
                    <option value="">{t('contact.form.subjectPlaceholder')}</option>
                    <option value="sales">{t('contact.form.subjects.sales')}</option>
                    <option value="support">{t('contact.form.subjects.support')}</option>
                    <option value="partnership">{t('contact.form.subjects.partnership')}</option>
                    <option value="other">{t('contact.form.subjects.other')}</option>
                  </select>
                  {errors.subject && <span className="text-red-500 text-xs mt-1">{errors.subject.message}</span>}
                </div>

                <div>
                  <label className="block text-sm font-medium text-gray-400 mb-2">{t('contact.form.message')}</label>
                  <textarea
                    {...register('message', { required: 'Message is required', minLength: { value: 10, message: "Message must be at least 10 characters" } })}
                    className="w-full px-4 py-3 bg-background border border-white/10 rounded-lg focus:border-accent-cyan focus:outline-none text-white h-32 resize-none transition-colors focus:bg-white/5"
                    placeholder={t('contact.form.messagePlaceholder')}
                  ></textarea>
                  {errors.message && <span className="text-red-500 text-xs mt-1">{errors.message.message}</span>}
                </div>

                <button
                  type="submit"
                  disabled={isSubmitting}
                  className="w-full py-4 bg-accent-cyan text-black font-bold rounded-lg hover:bg-accent-cyan/90 transition-all flex items-center justify-center disabled:opacity-50 disabled:cursor-not-allowed"
                >
                  {isSubmitting ? (
                    <span className="animate-pulse">{t('contact.form.sending')}</span>
                  ) : (
                    <>
                      {t('contact.form.send')} <Send className="ml-2 w-4 h-4" />
                    </>
                  )}
                </button>

                {submitStatus === 'success' && (
                  <motion.div
                    initial={{ opacity: 0, y: 10 }}
                    animate={{ opacity: 1, y: 0 }}
                    className="p-4 bg-green-500/10 border border-green-500/20 rounded-lg flex items-center text-green-500"
                  >
                    <CheckCircle className="w-5 h-5 mr-2" />
                    <span>{t('contact.form.success')}</span>
                  </motion.div>
                )}

                {submitStatus === 'error' && (
                  <motion.div
                    initial={{ opacity: 0, y: 10 }}
                    animate={{ opacity: 1, y: 0 }}
                    className="p-4 bg-red-500/10 border border-red-500/20 rounded-lg flex items-center text-red-500"
                  >
                    <AlertCircle className="w-5 h-5 mr-2" />
                    <span>{t('contact.form.error')}</span>
                  </motion.div>
                )}
              </form>
            </div>
          </motion.div>
        </div>
      </div>
    </div>
  );
};

export default Contact;
